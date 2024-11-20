"use client";

import { Me } from "@/actions/me";
import React, { createContext, useContext, useState, ReactNode } from "react";

interface MeContextType {
  me: Me;
}

const MeContext = createContext<MeContextType | undefined>(undefined);

export const MeProvider = ({
  children,
  initialMe,
}: {
  children: ReactNode;
  initialMe: Me;
}) => {
  const [me] = useState<Me>(initialMe);

  return <MeContext.Provider value={{ me }}>{children}</MeContext.Provider>;
};

export const useMe = (): Me => {
  const context = useContext(MeContext);
  if (context === undefined) {
    throw new Error("useMe must be used within a MeProvider");
  }
  return context.me;
};
