import { createLazyFileRoute } from "@tanstack/react-router";
import { MockGraphqlData } from "@/app/MockGraphqlData";

export const Route = createLazyFileRoute("/_app/mock-data")({
  component: MockGraphqlData,
}); 