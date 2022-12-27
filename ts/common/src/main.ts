import { Dog } from "./dog"
import { Human } from "./human"
import { Taro } from "./taro"

const dog = new Dog("Bow-wow!")
dog.shakeTail()

const human = new Human('Hanako')
human.shakeHands()

const taro = new Taro()
taro.greeting()

let list = ["this", "is", "a", "test"]
list.push("!")
console.log(list)

list = list.map(item => item.toUpperCase())
console.log(list)

let message = list.reduce((acc, value) => `${acc} ${value}`)
console.log(message)

function getFormattedValue(value: number, unit: string | null = "pt") {
  const _value = value.toFixed(2)
  if (unit === null) return `${_value}`
  return `${_value} ${unit.toUpperCase()}`
}

console.log(getFormattedValue(100))
console.log(getFormattedValue(100, 'kg'))

type Question = "exercise_habits" | "time_of_sleeping"
type Answer = "mighty" | "lot" | "few" | "entirely"
type User = {
  age: number
  name: string
  enquate: { [k in Question]?: Answer }
}
function registerUser(user: User) {
  console.log(user)
}

const maybeUser: User = {
  age: 26,
  name: "Taro",
  enquate: {
    exercise_habits: "entirely",
    time_of_sleeping: "few",
  }
}

registerUser(maybeUser)
registerUser({...{
  age: 26,
  name: "Taro",
  enquate: {},
}})

const exercise_habits = maybeUser.enquate["exercise_habits"]
console.log(exercise_habits)
