import { useState } from "react";
import { Outlet } from "react-router-dom";
import { AppShell, Burger, Group, ScrollArea, rem } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Sidebar } from "./Sidebar";
import { Header } from "./Header";

export function AppShellLayout() {
  const [opened, { toggle }] = useDisclosure();

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{
        width: 260,
        breakpoint: "sm",
        collapsed: { mobile: !opened },
      }}
      padding="md"
    >
      <AppShell.Header>
        <Group h="100%" px="md" justify="space-between">
          <Group>
            <Burger
              opened={opened}
              onClick={toggle}
              hiddenFrom="sm"
              size="sm"
            />
            <Header />
          </Group>
        </Group>
      </AppShell.Header>

      <AppShell.Navbar>
        <AppShell.Section grow component={ScrollArea}>
          <Sidebar
            onNavigate={() => {
              if (opened) toggle();
            }}
          />
        </AppShell.Section>
      </AppShell.Navbar>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}
