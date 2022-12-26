import { Creature } from "./creature"

export class Dog extends Creature {
  protected bark: string
  constructor(bark: string) {
    super(0, 4)
    this.bark = bark
  }

  protected barking() {
    return `${this.bark}! ${this.bark}!`
  }

  public shakeTail() {
    console.log(this.barking())
  }
}
