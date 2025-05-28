import { createFileRoute } from "@tanstack/react-router";
import { MockDataViewer } from "@/app/MockDataViewer";

export const Route = createFileRoute("/_app/mock/display")({
  component: MockDataViewer,
});
