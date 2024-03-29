let users = [{ id: 0, username: "exsample", password: "exsample" }];
export const db = {
  user: {
    async create({ data }) {
      let user = { ...data, id: users.length };
      users.push(user);
      return user;
    },
    async findUnique({ where: { username = undefined, id = undefined } }) {
      if (id !== undefined) {
        return users.find(user => user.id === id);
      } else {
        return users.find(user => user.username === username);
      }
    }
  }
};
