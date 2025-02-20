import { useEffect, useState } from "react";
import { createLazyFileRoute } from "@tanstack/react-router";
import { useNavigate } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/_app/")({
  component,
});

function component() {
  const navigate = useNavigate();

  useEffect(() => {
    navigate({ to: "/donation" });
  }, []);

  return (
    <div>
      <h1>Welcome to the Home Page</h1>
      <br /> 
    </div>
  );
}
