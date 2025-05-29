import { createFileRoute } from "@tanstack/react-router";
import { MockGraphqlGenerate } from "@/app/MockGraphqlGenerate";

export const Route = createFileRoute("/_app/mock/generate")({
  component: MockGraphqlGenerate,
});
