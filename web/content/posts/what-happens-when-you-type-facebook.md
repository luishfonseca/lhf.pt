+++
title = "So, what actually happens when you type facebook.com into the browser?"
date = 2024-12-11
description = """
I wrote this essay while preparing for a networking technical interview for an intern position at Meta. The information here is based on my understanding of the topic at the time, my bachelor's level course on computer networks, and public conference talks by Meta network engineers. No internal information was used in the writing of this essay, and it hasn't been modified to reflect what I was asked in the interview.

It covers all the relevant layers of the OSI model, L7 and L5 to L2, and explains how the protocols interact with each other to deliver the facebook homepage to the browser.
"""

[taxonomies]
tags = ["networking"]
keywords = ["faang", "meta", "facebook", "interview", "networking", "hsts", "https", "http", "tls", "dns", "udp", "tcp", "ipv4", "dhcp", "arp", "nat", "bgp", "igp", "anycast", "ipv6", "ndp", "ecmp"]
+++

I wrote this essay while preparing for a networking technical interview for an intern position at Meta. The information here is based on my understanding of the topic at the time, my bachelor's level course on computer networks, and public conference talks by Meta network engineers. No internal information was used in the writing of this essay, and it hasn't been modified to reflect what I was asked in the interview.

I got an offer.

To structure the essay, I'll be using the **OSI model**, and explaining how the protocols interact with each other to deliver the facebook homepage to the browser.

# L7 (HSTS, HTTP, TLS, Sockets, DNS and L7LBs)

Before even connecting to **facebook.com** the browser needs to pick the protocol, in this case that will be **HTTPS**. It is chosen over plain HTTP because facebook.com has **HSTS Preload** enabled, so the browser will enforce the HTTPS protocol.

This is preferred over allowing the first connection to be over HTTP and redirect that request to HTTPS, since that first interaction could be **hijacked by an impersonator**. 

**HTTPS is HTTP over a TLS tunnel.** The former, is responsible for transferring web content across machines. In this case, my laptop will be doing a GET request to a facebook webserver, and will receive a GET response including the HTML of the homepage. The browser uses the fetched content to send further GET requests for the JS, CSS and images and render the website.

The TLS tunnel, stands on top of the unsecured connection between my PC and facebook. It encrypts all communication between the two endpoints. The protocol is based on **Diffie-Hellman key exchange**. Both me and the webserver have a different secret, and we're able to exchange information over unsecured channels to compute a shared secret key, such that no one listening in on the channel could compute it. This key is then used to symmetrically encrypt all further communications.

To establish this tunnel, the protocol executes the following handshake. The client initiates by telling the server which options it supports and giving its nonce. The server then replies by choosing from those options and sending the public DH parameters and it's nonce. Additionally, it sends an asymmetric encryption signature of both nonces and DH parameters and a certificate chain, more on this later. The client checks the signature, and if it's good computes the shared secret. The secret key is derived from the shared secret and nonces.

Now the client gives the server the missing public info for it to also calculate the shared secret, and validates the previous steps by attaching a MAC (cryptographic, not the L2 address) of those messages computed with the secret key. Further messages will be encrypted by the client with the secret key. The server computes the shared secret and secret key, checks the received MAC, and replies with the MAC of the previous steps. The handshake is completed.

There's only one hole, if someone intercepts the handshake, they can modify the DH parameters and compromise all further traffic, so the authenticity of those must be assured. That motivates the signature and certificate chain the server sent. The client validates the signature public key with the first certificate in the chain, then if it's not a trusted certificate, it'll check the certificate with the next one until it finds one that as been installed on the browser as trusted.

At each step, the browser will make sure the certificate is valid and is not included in a CRL, and/or will contact OCSP servers to confirm that the certificate is valid. If at any point this fails, the handshake is interrupted.

The TLS tunnel is built on top of **TCP**, which is a L4 transport protocol. The application will connect to a stream socket provided by the operating system, but for that it will need to know its L4 address, i.e, the IP+Port pair. The port is picked based on the protocol, 443 for HTTPS, but for the IP it will need to use DNS to find the one corresponding to facebook.com

**DNS** is another L7 protocol responsible for converting domain names to addresses. Since my laptop has IPv4 and IPv6 connectivity it will send two requests. An A query and an AAAA query, for each version, to the DNS server configured on my computer, which is a local DNS adblocker and cache I have running on my home server. **The A and AAAA queries basically ask "What IP serves this domain?"**.

My laptop knows the L4 address of the DNS server, port 53 because DNS, and let's say 192.168.0.100, so it can open a datagram socket to it, to use the L4 transport protocol UDP. A datagram socket is enough here, as **lost datagrams simply result in a timeout and a repeated DNS query**.

Normally, my homeserver would have facebook.com cached, but if it doesn't find it forwards the request to Cloudflare's DNS resolver 1.1.1.1 in a similar fashion to how my laptop contacted my homeserver.

For the sake of argument, if 1.1.1.1 also doesn't have facebook.com cached, then it will try to find the server to ask by using multiple NS requests. **An NS query basically asks "What's the IP of the DNS server that can answer queries for this domain?"**. It will start by asking the root nameserver for the IP of the DNS server responsible for the .com TLD, then it will ask the .com DNS server for the DNS server responsible for facebook.com. Finally sending the A or AAAA query to that server.

The facebook.com DNS is also responsible for load balancing network infrastructure, so it will answer with the IP of a L7LB in a geographically close PoP. Cloudflare's 1.1.1.1 will then send this reply to my homeserver, which will send it to my laptop.

Facebook will answer both A and AAAA queries, so my laptop **will prefer IPv6**, and open the stream socket using that address. This socket will connect my laptop to the aforementioned PoP L7LB, which will then terminate TCP and TLS and proxy my HTTP requests through an already established TLS connection to a L7LB at one of their datacenters, which will finally forward my request to one of the web servers. The GET response will then be sent back to my laptop.

Terminating TCP and TLS at the L7LBs improves performance since the **handshakes will take place over a link with a much smaller RTT**.

# L4 (UDP, TCP and L4LBs)

When using the datagram socket for the DNS queries, the two endpoints are communicating with UDP. UDP is extremely bare-bones and doesn't add almost anything to the underlying L3, **just multiplexing: keeps track of correspondence between sockets and ports, and attaches the source and destination port to the packet**. As well as a checksum, to try to detect corruption.

When a datagram socket is created it picks a random high port on the client to use as the source port. Then, when the other end replies, it uses that port as the destination.

On the other hand, the behavior of the stream socket used for HTTPS is a lot more complex. The two endpoints communicate with TCP, which **in addition to multiplexing
also creates a reliable point-to-point stream**.

Each stream can be identified with the 4-tuple (source IP, source port, destination IP, destination port), and is established in a **3 step handshake**: SYN x, SYN y ACK x+1, ACK y+1. This initializes the sequence numbers at each end.

TCP ensures that all packets sent will be **delivered in the same order as they've been sent**. To do this with high performance, it keeps buffers at both ends, requesting missing segments to be resent.

It does this requests by always including the sequence number of the next expected segment in the ACK, this way if the other end ACKs get stuck the sender can notice that there is an hole in the buffer and fill it, without the next segments it already sent being wasted. **This is called fast retransmit, and is triggered at the third repeated ACK.**

The other mechanism triggering retransmission is a timer, this timer is set dynamically based on continuous sampling of the connection RTT. If the timer expires before receiving an ACK, the packet is retransmitted in an exponential backoff fashion, so to not overwhelm the link.

The other major responsibility of TCP is to **avoid congestion** of the links the stream passes through. It tries to fill it up as much as possible, without saturating the bottleneck. This is done by controlling the size of the sender's buffer. It starts small at the maximum size of a single segment and exponentially increases until it arrives at a threshold, after which it enters **congestion avoidance mode** and starts increasing following a **cubic function** with it's inflection point at the last retransmission event.

**These retransmission events are also responsible for adjusting the threshold.** When caused by a triple ACK, the threshold and buffer size are both set to half of the threshold. When caused by a timeout, indicated more serious congestion, the threshold is also halved but the buffer size is set back to 1 MSS.

It's also worth noting, that before traffic reaches facebook L7LBs, it goes through one of many L4LB, to further distribute the load. In order for the client endpoint to always talk to the same web server, both LBs pick the destination based on the 5-tuple (source IP, source port, destination IP, destination port, protocol) hash.

With this two-layer setup, even if the current L4LB used in some TCP connection dies, whichever replaces it will still hash to the same L7LB, **maintaining connectivity without bothering the user.**

# L3

During the whole process, there are 3 distinct network connections, each handled in strikingly different manners.

## Laptop <-> HomeServer (IPv4, DHCP, ARP and Fragmentation)

I'll start with my Laptop and HomeServer, since both of these are on my local LAN.

Previously, I've said that the HomeServer had IP 192.168.0.100, but we're still missing an IP for the laptop which can be 192.168.0.1. These addresses uniquely identify these two machines on my local network which is a /16 range, i.e, all IPs that start with 192.168. Anything outside this network is **sent to the default gateway**, i.e, my router, that has IP 192.168.0.254.

These definitions have not been configured manually, instead my laptop used **DHCP** when I connected to the wifi. Since it doesn't have an IP yet, during the request process the client uses 0.0.0.0, and servers broadcast their responses so they reach all machines. The client starts by broadcasting a discover and collecting offers from all DHCP servers in the network. Then, it selects one and broadcasts a request, which will be acknowledged by the offering server. From then on, my laptop can use it's DHCP IP.

Additional information can be added to the offer, such as the **network mask, default gateway, and DNS server**. This allows my laptop to fully configure it's network just from DHCP. In more advanced setups, **DHCP can even used for ZTP**, identifying the machine based on options in the discover, and including a configuration script in the offer through the bootfile option.

Finally then, my laptop needs to figure out the next destination to send its packet to. It will apply the network mask to the destination IP and compare to the local network. Since it's in the local network, it can **transmit the packet directly to the home server physical MAC address**.

It checks it's ARP cache, to see if it already knows the physical MAC address. If not, it **broadcasts an ARP request** by sending to the all Fs physical address, asking for the machine configured with the destination IP to reply with its physical MAC address. Then it can send the packet to the home server.

The final detail is that the laptop will fragment the packet into multiple L2 frames when transmitting, based on the MTU value. These will then be joined together on the other end of the L3 connection. If during the transmission, some intermediate networking hardware can't handle MTU sized frames, it'll further fragment the frames into smaller ones.

When replying, the homeserver will also find that it's a local IP, but won't need to send an ARP request since it'll will already have my laptop's physical MAC address cached from when it received the ARP request.

## HomeServer <-> Cloudflare (NAT, BGP, AnyCast and IGPs)

When the home server contacts cloudflare at 1.1.1.1, it'll find that it doesn't belong to the local network, and since there are no specific routes configured for how to reach it, it'll **forward the packet to the default gateway** 192.168.0.254. Note that the destination address of the IP packet is still 1.1.1.1, but it'll be sent to the physical MAC address corresponding to 192.168.0.254.

The packet will reach my router, which is connected to the ISP network, which for me is a black box, but I'll be making some guesses.

My home network is using a private network range, that can't be routed to the public internet. This is because my ISP only provides me with a **single IPv4 public IP**, with my router performing multiplexing through **NAT**. Let's say my router has public IP 200.0.0.1, it'll open a L4 port on it's outer interface, and match the inner ip and port to its outside ip and newly opened port. Packets going out will have their source replaced by the outside pair, and when packets come in destined to the outside pair, the destination is replaced by the inside pair.

This way, it will appear to 1.1.1.1 that it is talking to 200.0.0.1 directly, and not my laptop, but my laptop will receive all the traffic. **Which breaks the end-to-end principle of IP.**

After entering the ISP network, the packet will go through multiple routers until it reaches 1.1.1.1, and each will be configured with several routes. For each route, it'll will apply the route mask to the destination IP to see if it is contained. **After filtering, the most specific route is picked, i.e, longest prefix, and traffic is forwarded there.**

These routes can either be all inside the same AS, e.g two routers inside my ISP's network, or cross AS boundaries, e.g a peering ISP router at an IXP. AS is a network controlled by an organization, and all are assigned ASNs. The ASNs can be then used to find routes across multiple organizations.

The protocol used to exchange inter-AS routing information is B**GP, which gives strong control to each organization on which traffic should go through them**. Organizations statically peer their routers to other organizations, since BGP has no autodiscovery.

The routers start at an Idle state, moving to Connect and then Active while establishing a TCP tunnel between then, and finally Open Sent, Open Confirm, and Established, while exchanging information with an Open message. After establishing, the routers are ready to exchange or withdraw prefixes through Update messages.

When BGP advertises prefixes, it'll include the **full path of ASNs crossed to reach it, as it is a path-vector algorithm**. Then, a router can select a preferred route based on several attributes, **the most important of being local preference and path length**.

**Local preference** is the first evaluated of this two, and **allows administrators to give preference to certain routes**, which can be used to honor business relations on **outgoing traffic**. If two routes are tied, then one of the next attributes is **path length, which always prefers shorter paths**. This can be used to **control incoming traffic**, a router can **prepend its ASN multiple times to the advertisement**, discouraging other routers to pick it as the best route. Finally, if all attributes are tied, **router ID is the tie breaker**.

In this case, the route to 1.1.1.1 is special, since cloudflare isn't advertising that just for one machine, in fact multiple different machines have the 1.1.1.1 IP, injecting it into BGP in what's known as **anycast**. BGP will give preference to some route, which normally will be closer to the end user, allowing cloudflare to have **multiple machines serving 1.1.1.1** in the edge near all their users. In our case, 1.1.1.1 will be served at one of cloudflare's PoP near me.

This covers inter-AS, but packets still need to travel between the AS boundaries. In general, **BGP will inject it's routes into a IGP like ISIS or OSPF**, which will then will find the optimal route through the AS. Both this protocols are **link state**, meaning that they exchange information about the whole network topology, and then run Dijkstra to find the best path based on some cost function.

## Laptop <-> Facebook (IPv6, DHCPv6, NDP, ECMP)

When connecting from our laptop to facebook's servers, the biggest difference is the IPv6. Previously, we've been using IPv4, the legacy IP protocol. **IPv6 solves several problems, including IP exhaustion, and restores the end-to-end principle by removing the need for NAT.**

**IPv6 addresses are bigger**, 128b vs 32b, so there's enough address space to give each router a prefix instead of a single IP. Routers can then attribute an unique public IP to each of its inner network's devices.

This may be done with **stateless auto-configuration**, that generates an interface ID randomly or from the MAC address. First it concats it with link-local prefix, to create a private IP for the machine. With that IP, it can contact the router by sending it to the all-routers multicast address and requests its prefix. The router replies with the requested information, sending it to the **all-nodes multicast address**. Then my laptop can concat the prefix to its IID.

With stateless auto-configuration replacing DHCP, some additional features are missing, like getting the DNS servers. DHCPv6 can be used just for this, or to replace the whole process. The protocol is similar, except it **replaces broadcasting with multicasting to the dhcp-servers address**.

Another significant difference is the absence ARP, that has been **replaced with NDP**. Again, the protocol is similar, except it replaces broadcasting with **multicasting only to the nodes with a matching suffix to the destination IPv6 address**.

On facebook's end there's also something to note on how their BGP is configured. Their infrastructure is highly redundant, with multiple equally good paths. As such, they heavily use ECMP, which removes the router-id tie break from the decision, **allowing packets to the same prefix to be sent through multiple paths**, load balancing the load on the networking hardware.

While packet-by-packet load balancing is possible, this would put unnecessary stress on TCP and some UDP applications, requiring a lot more reordering. For this **ECMP decides on the route based on the 5-tuple hash**, so a TCP stream will also go through the same route. Additionally, in combination with AnyCast, this allows for **loadbalancing of the machine that handles the IP**. This explains how facebook has multiple redundant L4LB. 

# L2 (MAC, Switching, VLAN and CSMA)

Finally, we need to address how machines in the same local network communicate. This is handled by the NIC. It encapsulates each IP fragment in a frame, including the source and destination physical MAC addresses and a CRC for error checking.

The frame will then go across switches, which allows more that two machines to be attached to the same link. They'll **try to only forward to frame to the correct port**, but if that is unknown the frame is **broadcasted to all ports**. It learns as it goes, **registering in what port frames with a given source MACs arrive at**, so it can then only send frames with the same destination MAC there.

This behavior can be controlled by configuring **VLANs, that split the broadcast domain of the switch**. This way L2 frames can't be sent across VLAN boundaries. Switches ports can be assigned to just one VLAN, or NICs may tag some frames as part of a VLAN.

Having multiple NICs connected to the same shared medium creates the need to handle interference. **Carrier sensing multiple access** solves this, making NICs listen to the channel to make sure it's free before transmitting.

In wired connections, this is trivial, and is implemented as **CSMA/CD**. But in wireless networks, sender must **request to send** and reserve the channel for itself, as implemented in **CSMA/CA**.