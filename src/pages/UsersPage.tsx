import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getUsers, deleteUser, User } from '../services/users';
import './UsersPage.css';

export default function UsersPage() {
  const navigate = useNavigate();
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [error, setError] = useState('');

  const loadUsers = async () => {
    try {
      setLoading(true);
      setError('');
      const data = await getUsers({ limit: 100 });
      setUsers(data.users);
    } catch (err: any) {
      setError(err.message || 'Failed to load users');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadUsers();
  }, []);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this user? This action cannot be undone.')) return;
    try {
      setDeletingId(id);
      await deleteUser(id);
      setUsers(prev => prev.filter(u => u.id !== id));
    } catch (err: any) {
      alert(err.message || 'Failed to delete user');
    } finally {
      setDeletingId(null);
    }
  };

  const filteredUsers = users.filter(u => 
    u.first_name.toLowerCase().includes(search.toLowerCase()) || 
    u.last_name.toLowerCase().includes(search.toLowerCase()) ||
    u.email.toLowerCase().includes(search.toLowerCase()) ||
    u.username.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="users-root">
      {/* ── Page Header ── */}
      <div className="users-header">
        <div className="users-header__text">
          <h1 className="users-title">Users</h1>
          <p className="users-subtitle">Manage system users, roles, and access permissions.</p>
        </div>
        <div className="users-header__actions">
          <Link to="/users/new" className="users-btn users-btn--primary">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Add New User
          </Link>
        </div>
      </div>

      {error && (
        <div className="users-error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      {/* ── Main Card ── */}
      <div className="users-card">
        {/* Card Header (Search) */}
        <div className="users-card__head">
          <h2 className="users-card__title">User Directory</h2>
          <div className="users-search">
            <svg className="users-search__icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
            <input
              type="search"
              className="users-search__input"
              placeholder="Search by name, email, or username..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
        </div>

        {/* Table */}
        <div className="users-table-wrap">
          <table className="users-table">
            <thead>
              <tr>
                <th>User Details</th>
                <th>Username</th>
                <th>Email Address</th>
                <th>Role</th>
                <th>Status</th>
                <th className="users-th--center">Actions</th>
              </tr>
            </thead>
            <tbody>
              {loading && (
                <tr>
                  <td colSpan={6} className="users-table__loading">
                    <div className="users-spinner" />
                    Loading users...
                  </td>
                </tr>
              )}
              
              {!loading && filteredUsers.map((user) => (
                <tr key={user.id}>
                  {/* Name cell with avatar */}
                  <td>
                    <div className="users-name-cell">
                      <div className="users-avatar">
                        {user.first_name?.[0] ?? ''}{user.last_name?.[0] ?? ''}
                      </div>
                      <div className="users-name-info">
                        <span className="users-name">{user.first_name} {user.last_name}</span>
                        <span className="users-date">Added {new Date(user.created_at || Date.now()).toLocaleDateString()}</span>
                      </div>
                    </div>
                  </td>
                  <td className="users-td--muted">@{user.username}</td>
                  <td className="users-td--muted">{user.email}</td>
                  {/* Role */}
                  <td>
                    {user.role_id === '1' ? (
                      <span className="users-badge users-badge--indigo">Admin</span>
                    ) : (
                      <span className="users-badge users-badge--gray">User</span>
                    )}
                  </td>
                  {/* Status */}
                  <td>
                    {user.is_active ? (
                      <span className="users-badge users-badge--success">Active</span>
                    ) : (
                      <span className="users-badge users-badge--gray">Inactive</span>
                    )}
                  </td>
                  {/* Actions */}
                  <td className="users-td--center">
                    <div className="users-actions">
                      <button
                        className="users-action-btn users-action-btn--edit"
                        onClick={() => navigate(`/users/${user.id}/edit`)}
                        title="Edit User"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                      </button>
                      <button
                        className="users-action-btn users-action-btn--del"
                        onClick={() => handleDelete(user.id)}
                        disabled={deletingId === user.id}
                        title="Delete User"
                      >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/><path d="M9 6V4h6v2"/></svg>
                      </button>
                    </div>
                  </td>
                </tr>
              ))}

              {!loading && filteredUsers.length === 0 && (
                <tr>
                  <td colSpan={6} className="users-table__empty">
                    <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--c-slate-300)" strokeWidth="1.5"><path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                    <p>No users found</p>
                    {search ? (
                      <button className="users-empty-link" onClick={() => setSearch('')}>Clear search filter</button>
                    ) : (
                      <Link to="/users/new" className="users-empty-link">Add the first user →</Link>
                    )}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
      <div style={{ height: 32 }} />
    </div>
  );
}
