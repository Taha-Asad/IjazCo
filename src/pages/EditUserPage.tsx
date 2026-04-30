import { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getUser, updateUser, User } from '../services/users';
import './FormPage.css';

export default function EditUserPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  
  const [form, setForm] = useState({
    username: '',
    email: '',
    first_name: '',
    last_name: '',
    role_id: '',
  });

  useEffect(() => {
    if (id) loadUser();
  }, [id]);

  const loadUser = async () => {
    try {
      setLoading(true);
      setError('');
      const user: User = await getUser(id!);
      setForm({
        username: user.username || '',
        email: user.email || '',
        first_name: user.first_name || '',
        last_name: user.last_name || '',
        role_id: String(user.role_id) || '',
      });
    } catch (err: any) {
      setError(err.message || 'Failed to load user data.');
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setForm(prev => ({ ...prev, [e.target.name]: e.target.value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.username || !form.email || !form.role_id || !id) {
      setError('Please fill out all required fields.');
      return;
    }
    
    try {
      setSaving(true);
      await updateUser(id, form);
      navigate('/users');
    } catch (err: any) {
      setError(err.message || 'Failed to update user.');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="form-root">
        <div className="form-loading">Loading user data...</div>
      </div>
    );
  }

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Edit User</h1>
          <p className="form-subtitle">Update user information and access.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/users')}>
          Cancel
        </button>
      </div>

      {error && (
        <div className="form-error">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/></svg>
          {error}
        </div>
      )}

      <form className="form-card" onSubmit={handleSubmit}>
        <div className="form-grid">
          <div className="form-group">
            <label className="form-label">Username *</label>
            <input className="form-input" name="username" value={form.username} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Email Address *</label>
            <input className="form-input" type="email" name="email" value={form.email} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">First Name</label>
            <input className="form-input" name="first_name" value={form.first_name} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Last Name</label>
            <input className="form-input" name="last_name" value={form.last_name} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Role *</label>
            <select className="form-input" name="role_id" value={form.role_id} onChange={handleChange} required>
              <option value="">Select a role...</option>
              <option value="1">Admin</option>
              <option value="2">User</option>
            </select>
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/users')} disabled={saving}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={saving}>
            {saving ? 'Updating...' : 'Update User'}
          </button>
        </div>
      </form>
    </div>
  );
}
