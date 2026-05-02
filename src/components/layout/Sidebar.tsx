import { NavLink, Stack, Text, Divider, Box } from "@mantine/core";
import { useNavigate, useLocation } from "react-router-dom";
import {
  IconDashboard,
  IconUsers,
  IconBuilding,
  IconShield,
  IconCategory,
  IconUserCircle,
  IconTruck,
  IconPackage,
  IconStack2,
  IconReceipt,
  IconShoppingCart,
  IconContainer,
  IconChartBar,
  IconSettings,
  IconAlertTriangle,
  IconThumbUp,
} from "@tabler/icons-react";
import classes from "./Sidebar.module.css";

interface NavItemConfig {
  label: string;
  path: string;
  icon: React.ReactNode;
  section?: string;
}

const navItems: NavItemConfig[] = [
  {
    label: "Dashboard",
    path: "/dashboard",
    icon: <IconDashboard size={18} />,
    section: "Main",
  },
  {
    label: "Users",
    path: "/users",
    icon: <IconUsers size={18} />,
    section: "Administration",
  },
  {
    label: "Companies",
    path: "/companies",
    icon: <IconBuilding size={18} />,
    section: "Administration",
  },
  {
    label: "Roles",
    path: "/roles",
    icon: <IconShield size={18} />,
    section: "Administration",
  },
  {
    label: "Categories",
    path: "/categories",
    icon: <IconCategory size={18} />,
    section: "Administration",
  },
  {
    label: "Customers",
    path: "/customers",
    icon: <IconUserCircle size={18} />,
    section: "CRM",
  },
  {
    label: "Suppliers",
    path: "/suppliers",
    icon: <IconTruck size={18} />,
    section: "CRM",
  },
  {
    label: "Leads",
    path: "/leads",
    icon: <IconThumbUp size={18} />,
    section: "CRM",
  },
  {
    label: "Inventory",
    path: "/inventory",
    icon: <IconPackage size={18} />,
    section: "Inventory",
  },
  {
    label: "Low Stock",
    path: "/inventory/low-stock",
    icon: <IconAlertTriangle size={18} />,
    section: "Inventory",
  },
  {
    label: "Stock Levels",
    path: "/stock",
    icon: <IconStack2 size={18} />,
    section: "Inventory",
  },
  {
    label: "Stock Movements",
    path: "/stock/movements",
    icon: <IconStack2 size={18} />,
    section: "Inventory",
  },
  {
    label: "Sales Invoices",
    path: "/sales",
    icon: <IconReceipt size={18} />,
    section: "Transactions",
  },
  {
    label: "Purchase Orders",
    path: "/purchases",
    icon: <IconShoppingCart size={18} />,
    section: "Transactions",
  },
  {
    label: "Import Orders",
    path: "/imports",
    icon: <IconContainer size={18} />,
    section: "Transactions",
  },
  {
    label: "Reports",
    path: "/reports",
    icon: <IconChartBar size={18} />,
    section: "Analytics",
  },
  {
    label: "Settings",
    path: "/settings",
    icon: <IconSettings size={18} />,
    section: "System",
  },
];

interface SidebarProps {
  onNavigate?: () => void;
}

export function Sidebar({ onNavigate }: SidebarProps) {
  const navigate = useNavigate();
  const location = useLocation();

  const sections = [...new Set(navItems.map((item) => item.section))];

  return (
    <Box p="md">
      {sections.map((section) => (
        <Box key={section} mb="md">
          <Text size="xs" fw={600} c="dimmed" tt="uppercase" mb={4} px="sm">
            {section}
          </Text>
          <Stack gap={2}>
            {navItems
              .filter((item) => item.section === section)
              .map((item) => (
                <NavLink
                  key={item.path}
                  label={item.label}
                  leftSection={item.icon}
                  active={
                    location.pathname === item.path ||
                    (item.path !== "/dashboard" &&
                      location.pathname.startsWith(item.path))
                  }
                  onClick={() => {
                    navigate(item.path);
                    onNavigate?.();
                  }}
                  styles={{
                    root: { borderRadius: 8 },
                  }}
                />
              ))}
          </Stack>
          <Divider mt="xs" />
        </Box>
      ))}
    </Box>
  );
}
