import { describe, expect, it } from 'vitest';
import { hello } from './hello';

describe("hello", () => {
  it("message retrieve", () => {
    expect(hello()).toStrictEqual({"msg": "Hello World!"});
  })
})