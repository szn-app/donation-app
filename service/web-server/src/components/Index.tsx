import { useEffect } from "react";
import { useNavigate } from "@tanstack/react-router";

export function Component() {
  const navigate = useNavigate();

  useEffect(() => {
    navigate({ to: "/donation" });
  }, []);

  return (
    <div className="flex min-h-screen flex-col items-center justify-start pt-10">
      <h1 className="text-center text-3xl font-bold">Welcome</h1>
      <br />
    </div>
  );
}
