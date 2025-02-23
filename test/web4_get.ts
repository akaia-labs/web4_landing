const params = new URLSearchParams({
  "request.json": JSON.stringify({
    "account_id": null,
    "path": "/",
    "params": {},
    "query": {},
    "preloads": null,
  }),
});

fetch(
  `https://rpc.web4.testnet.page/account/web4tester.testnet/view/web4_get/${
    params.toString() ? "?" + params.toString() : ""
  }`,
)
  .then((response) => response.json())
  .then(({ contentType, body: base64Response }) => {
    if (contentType === "text/html; charset=UTF-8") {
      const decodedResponse = atob(base64Response);

      console.log(decodedResponse);
    } else {
      throw new Error(`Unsupported content type: ${contentType}`);
    }
  })
  .catch((error) => {
    console.error("Error:", error);
  });
