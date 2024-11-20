import { z } from "zod";

const loginSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(50),
  error: z.boolean().optional(),
});

const registerSchema = loginSchema
  .merge(
    z.object({
      confirmPassword: z.string(),
    })
  )
  .superRefine(({ confirmPassword, password }, ctx) => {
    if (confirmPassword !== password) {
      ctx.addIssue({
        code: "custom",
        message: "The passwords did not match",
        path: ["confirmPassword"],
      });
    }
  });

export { loginSchema, registerSchema };
