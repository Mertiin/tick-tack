import { Button } from "@/components/ui/button";
import { AuthStep } from "../types/enums";
import { Login } from "./login";
import { useState } from "react";
import { LoginHeader } from "./header";
import { Register } from "./register";

const Auth = () => {
  const [step, setStep] = useState<AuthStep>(AuthStep.login);

  return (
    <div className="flex flex-col gap-4 items-center">
      <LoginHeader
        title={step === AuthStep.login ? "Login" : "Register"}
        description={
          step === AuthStep.login
            ? "Enter your credentials to login"
            : "Create a new account"
        }
      />
      <div className="block w-[350px]">
        {step === AuthStep.login && <Login />}
        {step === AuthStep.register && <Register />}
      </div>
      <div className="grid justify-items-center">
        <span className="text-xs text-primary/60">OR</span>
        <Button
          type="button"
          variant={"link"}
          onClick={() => {
            setStep(
              step === AuthStep.login ? AuthStep.register : AuthStep.login
            );
          }}
        >
          {step === AuthStep.login && "Create an account"}
          {step === AuthStep.register && "Back to login"}
        </Button>
      </div>
    </div>
  );
};

export { Auth };
