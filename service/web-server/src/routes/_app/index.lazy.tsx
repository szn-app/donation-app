import { useEffect, useState } from "react";
import { createLazyFileRoute } from "@tanstack/react-router";
import { useNavigate } from "@tanstack/react-router";
import { Component } from "@/components/Index";

export const Route = createLazyFileRoute("/_app/")({
  component: Component,
});
