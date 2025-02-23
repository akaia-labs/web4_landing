import { get } from "../client/index.ts";

const main = () => {
  // TODO: pass inputs from command line
  get({ contractAccountId: "web4tester.testnet" }).then((response) => {
    console.log(response);
  }).catch((error) => {
    console.error("Error:", error);
  });
};

main();
