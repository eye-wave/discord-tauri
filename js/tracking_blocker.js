/** Blocks Discord analytics (/science, /track) and Sentry crash reports.
 * Stays well within what consumer adblockers do on the web client.
 */
(() => {
	const BLOCK_PATTERNS = [
		/\/api\/v\d+\/science\b/,
		/\/api\/v\d+\/track\b/,
		/sentry\.io/,
		/sentry\.discord\.com/,
		/crash\.discord\.com/,
	];

	const isBlocked = (url) => {
		try {
			return BLOCK_PATTERNS.some((re) => re.test(String(url)));
		} catch (_) {
			return false;
		}
	};

	const origFetch = window.fetch;
	window.fetch = function (input, init) {
		const url = typeof input === "string" ? input : input?.url || "";
		if (isBlocked(url)) {
			return Promise.resolve(
				new Response(null, { status: 204, statusText: "No Content" }),
			);
		}
		return origFetch.call(this, input, init);
	};

	const origOpen = XMLHttpRequest.prototype.open;
	XMLHttpRequest.prototype.open = function (method, url, ...rest) {
		this.__dt_blocked = isBlocked(url);
		return origOpen.call(this, method, url, ...rest);
	};

	const origSend = XMLHttpRequest.prototype.send;
	XMLHttpRequest.prototype.send = function (body) {
		if (this.__dt_blocked) {
			Object.defineProperty(this, "readyState", {
				value: 4,
				configurable: true,
			});
			Object.defineProperty(this, "status", { value: 204, configurable: true });
			Object.defineProperty(this, "responseText", {
				value: "",
				configurable: true,
			});
			setTimeout(() => {
				this.dispatchEvent(new Event("readystatechange"));
				this.dispatchEvent(new Event("load"));
				this.dispatchEvent(new Event("loadend"));
			}, 0);
			return;
		}
		return origSend.call(this, body);
	};

	if (navigator.sendBeacon) {
		const origBeacon = navigator.sendBeacon.bind(navigator);
		navigator.sendBeacon = (url, data) => {
			if (isBlocked(url)) return true;
			return origBeacon(url, data);
		};
	}
})();
