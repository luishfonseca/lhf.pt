+++
title = "Linux Networking Toolbox: Virtual Interfaces and Namespaces"
date = 2023-08-02
description = """

"""

[taxonomies]
tags = ["networking", "linux"]
+++

Inspired by an [amazing blog post from Tailscale](https://tailscale.com/blog/how-nat-traversal-works/), I decide to take on the challenge of developing my own barebones Tailscale clone - which I called [MagicMesh](https://github.com/luishfonseca/magicmesh).
Written in Rust, it would tackle a major gripe I had with their product, giving me complete control over my Wireguard keys rather than relying on their authentication system.

To make my vision a reality, I learned all about NAT, STUN, ICE and Wireguard. However, when it came time to implement my vision, I realized... I have no way to test it.

## The Challenge of Network Testing

When it came to testing my application in a network environment, I faced a significant challenge.
To properly evaluate its behavior, I would need two distinct LANs, accessible from a WAN through two distinct NATs. The goal was to establish a direct tunnel between clients in each LAN, using a coordinator server on the WAN to coordinate bypassing the NATs.

While I could have relied on existing residential networking infrastructure, this approach demanded access to two separate buildings with an internet connections and a VPS with a public IP - a far from practical option. Alternatively, building a physical lab with networking hardware (more exactly, two routers and a switch) was an option, but the expenses involved made it less appealing.

## Embracing Virtualization

Determined to find a more cost-effective solution, I explored the realm of virtualization. Linux containers utilize network namespaces to isolate their networks from the host, such behavior would allow me to create distinct network environments on my laptop.

Despite not ultimately using containers, the idea of network namespaces prompted further exploration of its simplicity and power. Through the command `ip netns exec`, I could run any command inside a network namespace, giving me total control over what interfaces and routes were available to it.

```bash
# Create a new network namespace named ns1
sudo ip netns add ns1

# Run `ip a` inside the namespace
sudo ip netns exec ns1 ip a
```

As expected, there is not much to see as this namespace hasn't been connected to anything.

## VEth: The Virtual Ethernet Cable

To connect two network namespaces, I needed a virtual cable and the `veth` interface offered the perfect solution. This virtual device comes in pairs, when a packet is sent through one end, it is received by the other. By moving one end of the pair to a different namespace, I could connect two namespaces.

```bash
# Create a veth pair on the ns1 namespace,
# with one end named veth-ns1 and the other peer-ns1
sudo ip netns exec ns1 ip link add veth-ns1 type veth peer name peer-ns1

# Create a second namespace, ns2
sudo ip netns add ns2

# Move the peer-ns1 end to the ns2 namespace
sudo ip netns exec ns1 ip link set peer-ns1 netns ns2

# Bring up the interfaces
sudo ip netns exec ns1 ip link set veth-ns1 up
sudo ip netns exec ns2 ip link set peer-ns1 up

# Run `ip a` inside ns2
sudo ip netns exec ns2 ip a
```

As expected, the `peer-ns1` interface is now accessible within the `ns2` namespace.

To test the connection, I assigned IP addresses from the `172.16.0.0/24` subnet, a reserved range for private networks. The `ns1` end received `172.16.0.1`, while the `ns2` end received `172.16.0.2`.
Pinging `ns2` from `ns1` validated the success of our veth pair.

```bash
# Assign IP addresses to the interfaces
sudo ip netns exec ns1 ip addr add 172.16.0.1/24 dev veth-ns1
sudo ip netns exec ns2 ip addr add 172.16.0.2/24 dev peer-ns1

# Ping address on ns2's interface from ns1
sudo ip netns exec ns1 ping 172.16.0.2
```

With our virtual cable in place, we can now connect two network namespaces. But our virtual lab is still missing switches and routers.

## Bridge: The Virtual Switch

Connecting more that two interfaces requires a virtual switch is required, and this role is effectively fulfilled by the `bridge` interface. Just like a physical switch, the `bridge` operates at Layer 2 and forwards Ethernet frames based on MAC addresses. Unlike a router that keeps network segments isolated, a bridge unites all interfaces, connecting them as a single network.

Expanding our setup to include a third namespace, `ns3`, we establish a new veth pair connecting it to `ns2`, where the bridge will be created. This arrangement effectively connects all three namespaces.

```bash
# Create a third namespace, ns3
sudo ip netns add ns3

# Create a veth pair on the ns3 namespace,
# with one end named veth-ns3 and the other peer-ns3
sudo ip netns exec ns3 ip link add veth-ns3 type veth peer name peer-ns3

# Move the peer-ns3 end to the ns2 namespace
sudo ip netns exec ns3 ip link set peer-ns3 netns ns2

# Bring up the interfaces
sudo ip netns exec ns3 ip link set veth-ns3 up
sudo ip netns exec ns2 ip link set peer-ns3 up
```

At this point, we have two cables going to `ns3`, but they are not connected to anything in that namespace. To address this, we create the bridge and connect the network interfaces to it.

```bash
# Create a bridge named br0
sudo ip netns exec ns2 ip link add name br0 type bridge
sudo ip netns exec ns2 ip link set br0 up

# Connect the peer-ns1 and peer-ns3 interfaces to the bridge
sudo ip netns exec ns2 ip link set peer-ns1 master br0
sudo ip netns exec ns2 ip link set peer-ns3 master br0
```

Next, we assign an IP address, `172.16.0.100`, to the `br0` interface in the `ns2` namespace, enabling us to ping it from both `ns1` and `ns3`. Additionally, we reassign `172.16.0.2` to the `ns3` end of the newly created veth pair, for consistency.

```bash
# Clear the IP address from the peer-ns1 interface
sudo ip netns exec ns2 ip addr del 172.16.0.2/24 dev peer-ns1

# Assign the IP addresses
sudo ip netns exec ns3 ip addr add 172.16.0.2/24 dev veth-ns3
sudo ip netns exec ns2 ip addr add 172.16.0.100/24 dev br0

# Ping the bridge from ns1
sudo ip netns exec ns1 ping 172.16.0.100

# Ping ns3 from ns1
sudo ip netns exec ns1 ping 172.16.0.2
```

With the bridge in place, we can now connect any number of namespaces. However, the bridge does not provide any routing capabilities, so all interfaces are still on the same network segment. To address this, we need a router.

## Missing Piece: Building the Router

To complete our virtual lab, we need two LANs, each with its own router. While routers can be hardware-based with specialized silicon, they can also be implemented in software. The Linux kernel provides all the necessary tools to build a router.

We will start by enabling IP forwarding on the `ns1` namespace. This allows the kernel to forward packets between interfaces, effectively turning the namespace into a router.

```bash
# Enable IP forwarding on ns1
sudo ip netns exec ns1 sysctl -w net.ipv4.ip_forward=1
```

Next, we'll create one more namespace, `ns4`, to serve as one of our LAN environments. It will be connected to `ns1` via a veth pair, just like `ns2` and `ns3` were.

```bash
# Create a fourth namespace, ns4
sudo ip netns add ns4

# Create a veth pair on the ns4 namespace,
# with one end named veth-ns4 and the other peer-ns4
sudo ip netns exec ns4 ip link add veth-ns4 type veth peer name peer-ns4

# Move the peer-ns4 end to the ns1 namespace
sudo ip netns exec ns4 ip link set peer-ns4 netns ns1

# Bring up the interfaces
sudo ip netns exec ns4 ip link set veth-ns4 up
sudo ip netns exec ns1 ip link set peer-ns4 up
```

With the interfaces in place, we can proceed to create the router. The `ns1` namespace contains the `peer-ns4` interface (the LAN interface) and the `peer-ns1` interface (the WAN interface). Packets arriving at the LAN interface should be forwarded to the WAN interface, but not vice versa. A connection must have been initiated from the LAN for the router to forward packets from the WAN to the LAN.

To achieve this, we will use `nftables` to set up the necessary forwarding rules:

```bash
# Add nftable rules to forward packets from the LAN to the WAN
sudo ip netns exec ns1 nft add table inet filter
sudo ip netns exec ns1 nft add chain inet filter forward { type filter hook forward priority 0 \; }
sudo ip netns exec ns1 nft add rule inet filter forward iif veth-ns1 oif peer-ns4 ct state related,established accept
```

Additionally, we must enable NAT on the router so that packet display the router's public IP address when they reach other namespaces. This is done by adding a `masquerade` rule.

```bash
# Add nftable rules to enable NAT
sudo ip netns exec ns1 nft add table nat
sudo ip netns exec ns1 nft add chain nat postrouting { type nat hook postrouting priority 100 \; }
sudo ip netns exec ns1 nft add rule nat postrouting masquerade random,persistent
```

To complete the configuration, we assign IP addresses to the `ns1` and `ns4` interfaces, utilizing the `10.1.0.0/24` range for the LAN, with `10.1.0.1` designated for `ns4`, and `10.1.0.254` designated to the LAN interface of the router. The WAN interface of the router had previously received `172.16.0.1`. After setting the default gateway for `ns4` as the router, pings should reach every other namespace.

```bash
# Assign IP addresses to the interfaces
sudo ip netns exec ns1 ip addr add 10.1.0.254/24 dev peer-ns4
sudo ip netns exec ns4 ip addr add 10.1.0.1/24 dev veth-ns4

# Set the default gateway on ns4
sudo ip netns exec ns4 ip route add default via 10.1.0.254 dev veth-ns4

# Ping ns2 from ns4
sudo ip netns exec ns4 ping 172.16.0.100
```

To test NAT functionality, we utilize  netcat. Starting a verbose netcat listener on `ns2` and connecting to it from `ns4`, the listener should display the IP address of the connecting client, which should be the routers public IP address.

```bash
# Start a netcat listener on ns2
sudo ip netns exec ns2 nc -l -v 1234

# On another terminal!!!
# Connect to the listener from ns4
sudo ip netns exec ns4 nc 172.16.0.100
```

To complete our virtual lab, we'll create a second LAN, ns5, and replicate the steps previously executed for ns4.

```bash
# Enable IP forwarding on ns3
sudo ip netns exec ns1 sysctl -w net.ipv4.ip_forward=1

# Create a fifth namespace, ns5
sudo ip netns add ns5

# Create a veth pair on the ns5 namespace,
# with one end named veth-ns5 and the other peer-ns5
sudo ip netns exec ns5 ip link add veth-ns5 type veth peer name peer-ns5

# Move the peer-ns5 end to the ns3 namespace
sudo ip netns exec ns5 ip link set peer-ns5 netns ns3

# Bring up the interfaces
sudo ip netns exec ns5 ip link set veth-ns5 up
sudo ip netns exec ns3 ip link set peer-ns5 up

# Add nftable rules to forward packets from the LAN to the WAN
sudo ip netns exec ns3 nft add table inet filter
sudo ip netns exec ns3 nft add chain inet filter forward { type filter hook forward priority 0 \; }
sudo ip netns exec ns3 nft add rule inet filter forward iif veth-ns3 oif peer-ns5 ct state related,established accept

# Add nftable rules to enable NAT
sudo ip netns exec ns3 nft add table nat
sudo ip netns exec ns3 nft add chain nat postrouting { type nat hook postrouting priority 100 \; }
sudo ip netns exec ns3 nft add rule nat postrouting masquerade random,persistent

# Assign IP addresses to the interfaces
sudo ip netns exec ns3 ip addr add 10.2.0.254/24 dev peer-ns5
sudo ip netns exec ns5 ip addr add 10.2.0.1/24 dev veth-ns5

# Set the default gateway on ns5
sudo ip netns exec ns5 ip route add default via 10.2.0.254 dev veth-ns5
```