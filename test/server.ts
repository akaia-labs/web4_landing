import { get } from "../client/index.ts";

export default {
	fetch(request) {
		const decodedUrl = new URL(request.url);

		return get({
			network: "mainnet",
			contractAccountId: "akaia.near",
			path: decodedUrl.pathname,
		}).then((response) => {
			return new Response(response);
		});
	},
} satisfies Deno.ServeDefaultExport;
