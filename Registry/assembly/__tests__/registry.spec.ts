import { addGameAddress, getGameAddresses, getGameAddress } from "../main";
import { GameAddress, gameAddresses } from "../model";

const gameAddress = new GameAddress("address", "hello world");

describe("gameAddress tests", () => {
  afterEach(() => {
    while (gameAddresses.length > 0) {
      gameAddresses.pop();
    }
  });

  it("adds a gameAddress", () => {
    addGameAddress("address", "hello world");
    expect(gameAddresses.length).toBe(1, "should only contain one gameAddress");
    expect(gameAddresses[0]).toStrictEqual(
      gameAddress,
      'gameAddress should be "hello world"'
    );
  });

  it("retrieves gameAddresses", () => {
    addGameAddress("address1", "hello world");
    addGameAddress("address2", "hello world");
    addGameAddress("address3", "hello world");
    addGameAddress("address4", "hello world");
    const gameAddressesArr = getGameAddresses(1);
    expect(gameAddressesArr.length).toBe(3, "should be one gameAddress");

    const tGA = new GameAddress("address4", "hello world");
    expect(gameAddressesArr).toIncludeEqual(
      tGA,
      "gameAddresses should include:\n" + tGA.toJSON()
    );
  });

  it("retrieves gameAddress by id", () => {
    addGameAddress("address", "hello world");
    const _gameAddresses = getGameAddress(0);
    expect([_gameAddresses]).toIncludeEqual(
      gameAddress,
      "gameAddresses should include:\n" + gameAddress.toJSON()
    );
  });
});
