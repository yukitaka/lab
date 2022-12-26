import { Creature } from "./creature"

export class Human extends Creature {
  protected name: string
  constructor(name: string) {
    super(2, 2)
    this.name = name
  }

  protected greet() {
    return `Hello! I'm ${this.name}.`
  }

  public shakeHands() {
    console.log(this.greet())
  }
}
