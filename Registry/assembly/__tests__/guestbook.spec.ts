import { addGameAddress, getGameAddresses, getGameAddress } from "../main";
import { GameAddress, gameAddresses } from "../model";
import { VMContext, Context, u128 } from "near-sdk-as";

function createGameAddress(contractAddress: string): GameAddress {
  return new GameAddress(contractAddress);
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

  it("retrieves gameAddresses", () => {
    addGameAddress("hello world");
    const gameAddressesArr = getGameAddresses();
    expect(gameAddressesArr.length).toBe(1, "should be one gameAddress");
    expect(gameAddressesArr).toIncludeEqual(
      gameAddress,
      "gameAddresses should include:\n" + gameAddress.toJSON()
    );
  });

  it("retrieves gameAddress by id", () => {
    addGameAddress("hello world");
    const _gameAddresses = getGameAddress(0);
    expect(_gameAddresses).toIncludeEqual(
      gameAddress,
      "gameAddresses should include:\n" + gameAddress.toJSON()
    );
  });

  it("only show the last 20 gameAddresses", () => {
    addGameAddress("hello world");
    const newAddresses: GameAddress[] = [];
    for (let i: i32 = 0; i < 20; i++) {
      const text = "gameAddress #" + i.toString();
      newAddresses.push(createGameAddress(text));
      addGameAddress(text);
    }
    const gameAddresses = getGameAddresses();
    log(gameAddresses.slice(7, 10));
    expect(gameAddresses).toStrictEqual(
      newAddresses,
      "should be the last ten gameAddresses"
    );
    expect(gameAddresses).not.toIncludeEqual(
      gameAddress,
      "shouldn't contain the first element"
    );
  });
});
