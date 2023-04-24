<script lang="ts">
	import { page } from '$app/stores';
	import { browser, dev } from '$app/environment';

	import { onCLS, onFCP, onFID, onLCP, onTTFB, type Metric } from 'web-vitals';

	const vitalsUrl = 'https://vitals.vercel-analytics.com/v1/vitals';
	const analyticsId = import.meta.env.VERCEL_ANALYTICS_ID;

	const getConnectionSpeed = () => {
		return 'connection' in navigator &&
			navigator['connection'] && // @ts-ignore
			'effectiveType' in navigator['connection']
			? (navigator['connection'].effectiveType as string)
			: '';
	};

	const sendToAnalytics = (metric: Metric) => {
		const body = {
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
		};

		if (dev) {
			console.log('[Web Vitals]', metric.name, JSON.stringify(body, null, 2));
		} else {
			navigator.sendBeacon(
				vitalsUrl,
				new Blob([new URLSearchParams(body).toString()], {
					type: 'application/x-www-form-urlencoded'
				})
			);
		}
	};

	if (browser && analyticsId) {
		onCLS(sendToAnalytics);
		onFCP(sendToAnalytics);
		onFID(sendToAnalytics);
		onLCP(sendToAnalytics);
		onTTFB(sendToAnalytics);
	}
</script>
