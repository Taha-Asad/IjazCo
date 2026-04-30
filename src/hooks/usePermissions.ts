import { useAuthStore } from "../store";

export type ResourceAction = "create" | "read" | "update" | "delete" | "*";

export function usePermissions() {
  const { user } = useAuthStore();

  const can = (resource: string, action: ResourceAction): boolean => {
    if (!user?.permissions) return false;
    const perms = user.permissions[resource];
    if (!perms) return false;
    return perms.includes("*") || perms.includes(action);
  };

  const canAny = (resource: string, actions: ResourceAction[]): boolean => {
    return actions.some((action) => can(resource, action));
  };

  const canAll = (resource: string, actions: ResourceAction[]): boolean => {
    return actions.every((action) => can(resource, action));
  };

  const isAdmin = (): boolean => user?.role_name?.toLowerCase() === "admin";

  const isRole = (...roles: string[]): boolean =>
    roles.some((r) => r.toLowerCase() === user?.role_name?.toLowerCase());

  return { can, canAny, canAll, isAdmin, isRole };
}
