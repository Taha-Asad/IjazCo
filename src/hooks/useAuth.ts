import { useAuthStore } from "@/store/authStore";

export function useAuth() {
  const { user, isAuthenticated, logout } = useAuthStore();

  const hasPermission = (resource: string, action: string): boolean => {
    if (!user?.permissions) return false;
    const resourcePerms = user.permissions[resource];
    if (!resourcePerms) return false;
    return resourcePerms.includes(action) || resourcePerms.includes("*");
  };

  const isAdmin = (): boolean => {
    return user?.role_name?.toLowerCase() === "admin";
  };

  const isRole = (roleName: string): boolean => {
    return user?.role_name?.toLowerCase() === roleName.toLowerCase();
  };

  return {
    user,
    isAuthenticated,
    logout,
    hasPermission,
    isAdmin,
    isRole,
  };
}
