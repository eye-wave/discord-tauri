/** Watches document.title for Discord's unread/mention markers and forwards the
 * state to native code so the tray icon and dock tile can display a badge.
 *   "(N) ..."  -> N unread, "badge:N"
 *   "• ..."    -> mentions without a count, "badge:dot"
 *   otherwise  -> "badge:0"
 */
(() => {
	let last = null;
	const send = () => {
		const t = document.title || "";
		const m = t.match(/^\((\d+)\)/);
		const next = m ? "badge:" + m[1] : /^•/.test(t) ? "badge:dot" : "badge:0";
		if (next === last) return;
		last = next;
		window?.ipc?.postMessage(next);
	};
	// Title element appears mid-parse on Discord; wait for it before observing.
	const attach = () => {
		const t = document.querySelector("title");
		if (!t) return false;
		new MutationObserver(send).observe(t, {
			childList: true,
			characterData: true,
			subtree: true,
		});
		send();
		return true;
	};
	if (!attach()) {
		const waiter = new MutationObserver(() => {
			if (attach()) waiter.disconnect();
		});
		waiter.observe(document.documentElement, {
			childList: true,
			subtree: true,
		});
	}
})();
