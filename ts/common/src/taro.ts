import { Human } from "./human"

export class Taro extends Human {
  constructor() {
    super('Taro')
  }

  public greeting() {
    console.log(this.greet())
  }
}
