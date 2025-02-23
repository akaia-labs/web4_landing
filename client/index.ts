type SupportedNearProtocolNetwork = "testnet" | "mainnet";

type GetParams = {
  network?: SupportedNearProtocolNetwork;
  contractAccountId: string;
  path?: string;
};

export const get = (
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

  return fetch(
    `https://rpc.web4.${
      network === "mainnet" ? "near" : network
    }.page/account/${contractAccountId}/view/web4_get?${params.toString()}`,
  )
    .then((response) => response.json())
    .then(({ contentType, body: base64Response }) => {
      if (contentType === "text/html; charset=UTF-8") {
        return atob(base64Response);
      } else {
        throw new Error(`Unsupported content type: ${contentType}`);
      }
    })
    .catch((error) => {
      const errorMessage = error instanceof Error
        ? error.message
        : String(error);

      return errorMessage;
    });
};
