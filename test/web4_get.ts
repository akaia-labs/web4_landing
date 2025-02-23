type SupportedNearProtocolNetwork = "testnet" | "mainnet";

type GetParams = {
  network?: SupportedNearProtocolNetwork;
  contractAccountId: string;
  path?: string;
};

const get = (
  { network = "testnet", contractAccountId, path = "/" }: GetParams,
) => {
  const params = new URLSearchParams({
    "request.json": JSON.stringify({
      account_id: null,
      path,
      params: {},
      query: {},
      preloads: null,
    }),
  });

  fetch(
    `https://rpc.web4.${
      network === "mainnet" ? "near" : network
    }.page/account/${contractAccountId}/view/web4_get?${params.toString()}`,
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
};

const main = () => {
  // TODO: pass inputs from command line
  get({ contractAccountId: "web4tester.testnet" });
};

main();
