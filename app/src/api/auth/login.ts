const login = async (email: string, password: string) => {
  // login logic
  const result = fetch("http://localhost:3001/api/auth/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email, password }),
  }).then((res) => res.json());

  return result;
};

export { login };
