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
  type strnum = string & never

  const never_check: strnum = "never" as never
  console.log(never_check)

  const intersection: Intersection = {
    name: "test",
    age: 1,
    address: "address"
  } // all property is need.
  console.log(intersection)

  type Union = User | Address
  const union: Union = {
    name: "test",
    age: 1
  }
  console.log(union)
}
