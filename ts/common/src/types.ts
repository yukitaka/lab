export function types() {
  type User = {
    name: string
    age: number
  }

  type Address = {
    name: string
    address: string
  }

  type Intersection = User & Address

  const intersection: Intersection = {
    name: "test",
    age: 1,
    address: "address"
  } // all property is need.
  console.log(intersection)
}
