import { createFileRoute } from "@tanstack/react-router";
import { MockGraphqlData } from "@/app/MockGraphqlData";

// TODO:
export const Route = createFileRoute("/_app/mock/display")({
  component: MockGraphqlData,
});
