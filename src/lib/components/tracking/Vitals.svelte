<script lang="ts">
	import { page } from '$app/stores';
	import { browser, dev } from '$app/environment';

	import { onCLS, onFCP, onFID, onINP, onLCP, onTTFB, type Metric } from 'web-vitals';

	const vitalsUrl = 'https://vitals.vercel-analytics.com/v1/vitals';
	const analyticsId = import.meta.env.VERCEL_ANALYTICS_ID;

	const getConnectionSpeed = () => {
		return 'connection' in navigator &&
			(navigator as any)['connection'] &&
			'effectiveType' in (navigator as any)['connection']
			? ((navigator as any)['connection'].effectiveType as string)
			: '';
	};

	const sendToAnalytics = (metric: Metric) => {
		const body = new Blob(
			[
				new URLSearchParams({
					dsn: analyticsId,
					id: metric.id,
					href: location.href,
					event_name: metric.name,
					value: metric.value.toString(),
					speed: getConnectionSpeed(),
					page: Object.entries($page.params).reduce(
						(acc, [k, v]) => acc.replace(v, `[${k}]`),
						$page.url.pathname
					)
				}).toString()
			],
			{
				type: 'application/x-www-form-urlencoded'
			}
		);

		if (dev) {
			console.log('[Web Vitals]', metric.name, JSON.stringify(body, null, 2));
		} else {
			const send = navigator.sendBeacon && navigator.sendBeacon.bind(navigator);

			const fallbackSend = () => {
				fetch(vitalsUrl, {
					body,
					method: 'POST',
					credentials: 'omit',
					keepalive: true
				}).catch(console.error);
			};

			try {
				send!(vitalsUrl, body) || fallbackSend();
			} catch (e) {
				fallbackSend();
			}
		}
	};

	if (browser && analyticsId) {
		onCLS(sendToAnalytics);
		onFCP(sendToAnalytics);
		onFID(sendToAnalytics);
		onINP(sendToAnalytics);
		onLCP(sendToAnalytics);
		onTTFB(sendToAnalytics);
	}
</script>
