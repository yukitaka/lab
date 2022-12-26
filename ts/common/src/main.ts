import { Dog } from "./dog"
import { Human } from "./human"
import { Taro } from "./taro"

const dog = new Dog("Bow-wow!")
dog.shakeTail()

const human = new Human('Hanako')
human.shakeHands()

const taro = new Taro()
taro.greeting()
