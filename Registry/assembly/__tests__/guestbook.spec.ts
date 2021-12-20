import { addGameAddress, getGameAddresses } from "../main";
import { GameAddress, gameAddresses } from "../model";
import { VMContext, Context, u128 } from "near-sdk-as";

function createGameAddress(text: string): GameAddress {
  return new GameAddress(text);
}

const gameAddress = createGameAddress("hello world");

describe("gameAddress tests", () => {
  afterEach(() => {
    while (gameAddresses.length > 0) {
      gameAddresses.pop();
    }
  });

  it("adds a gameAddress", () => {
    addGameAddress("hello world");
    expect(gameAddresses.length).toBe(1, "should only contain one gameAddress");
    expect(gameAddresses[0]).toStrictEqual(
      gameAddress,
      'gameAddress should be "hello world"'
    );
  });

  it("adds a premium gameAddress", () => {
    VMContext.setAttached_deposit(u128.from("10000000000000000000000"));
    addGameAddress("hello world");
    const gameAddressAR = getGameAddresses();
    expect(gameAddressAR[0].premium).toStrictEqual(true, "should be premium");
  });

  it("retrieves gameAddresses", () => {
    addGameAddress("hello world");
    const gameAddressesArr = getGameAddresses();
    expect(gameAddressesArr.length).toBe(1, "should be one gameAddress");
    expect(gameAddressesArr).toIncludeEqual(
      gameAddress,
      "gameAddresses should include:\n" + gameAddress.toJSON()
    );
  });

  it("only show the last 10 gameAddresses", () => {
    addGameAddress("hello world");
    const newMessages: GameAddress[] = [];
    for (let i: i32 = 0; i < 10; i++) {
      const text = "gameAddress #" + i.toString();
      newMessages.push(createGameAddress(text));
      addGameAddress(text);
    }
    const gameAddresses = getGameAddresses();
    log(gameAddresses.slice(7, 10));
    expect(gameAddresses).toStrictEqual(
      newMessages,
      "should be the last ten gameAddresses"
    );
    expect(gameAddresses).not.toIncludeEqual(
      gameAddress,
      "shouldn't contain the first element"
    );
  });
});

describe("attached deposit tests", () => {
  beforeEach(() => {
    VMContext.setAttached_deposit(u128.fromString("0"));
    VMContext.setAccount_balance(u128.fromString("0"));
  });

  it("attaches a deposit to a contract call", () => {
    log("Initial account balance: " + Context.accountBalance.toString());

    addGameAddress("hello world");
    VMContext.setAttached_deposit(u128.from("10"));

    log("Attached deposit: 10");
    log("Account balance after deposit: " + Context.accountBalance.toString());

    expect(Context.accountBalance.toString()).toStrictEqual(
      "10",
      "balance should be 10"
    );
  });
});
