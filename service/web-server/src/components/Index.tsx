import { useEffect } from "react";
import { useNavigate } from "@tanstack/react-router";

export function Component() {
  const navigate = useNavigate();

  useEffect(() => {
    // navigate({ to: "/donation" });
  }, []);

  return (
    <div>
      <h1>WELCOME</h1>
      <br />
    </div>
  );
}
