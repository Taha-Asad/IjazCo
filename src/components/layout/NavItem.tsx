import { NavLink, ThemeIcon } from "@mantine/core";
import { useNavigate, useLocation } from "react-router-dom";

interface NavItemProps {
  label: string;
  path: string;
  icon: React.ReactNode;
  color?: string;
  onClick?: () => void;
  badge?: number;
}

export function NavItem({
  label,
  path,
  icon,
  color = "erp-green",
  onClick,
  badge,
}: NavItemProps) {
  const navigate = useNavigate();
  const location = useLocation();

  const isActive =
    path === "/dashboard"
      ? location.pathname === path
      : location.pathname.startsWith(path);

  return (
    <NavLink
      label={label}
      leftSection={
        <ThemeIcon
          size="sm"
          variant={isActive ? "filled" : "subtle"}
          color={color}
          radius="sm"
        >
          {icon}
        </ThemeIcon>
      }
      active={isActive}
      onClick={() => {
        navigate(path);
        onClick?.();
      }}
      rightSection={
        badge ? (
          <span
            style={{
              background: "var(--mantine-color-red-6)",
              color: "#fff",
              borderRadius: 999,
              padding: "0 6px",
              fontSize: 11,
              fontWeight: 700,
            }}
          >
            {badge}
          </span>
        ) : null
      }
      styles={{
        root: {
          borderRadius: 8,
          fontWeight: isActive ? 600 : 400,
        },
      }}
    />
  );
}
