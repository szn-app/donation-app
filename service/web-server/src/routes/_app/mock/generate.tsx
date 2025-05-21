import { createFileRoute } from "@tanstack/react-router";
import { MockGraphqlData } from "@/app/MockGraphqlData";

export const Route = createFileRoute("/_app/mock/generate")({
  component: MockGraphqlData,
}); 