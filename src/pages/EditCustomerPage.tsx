import { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getCustomer, updateCustomer, Customer } from '../services/customers';
import './FormPage.css';

export default function EditCustomerPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  
  const [form, setForm] = useState({
    name: '',
    email: '',
    phone: '',
    address: '',
    city: '',
    country: '',
    tax_number: '',
    credit_limit: '',
    is_active: true,
  });

  useEffect(() => {
    if (id) loadData();
  }, [id]);

  const loadData = async () => {
    try {
      setLoading(true);
      setError('');
      const data: Customer = await getCustomer(id!);
      
      setForm({
        name: data.name || '',
        email: data.email || '',
        phone: data.phone || '',
        address: data.address || '',
        city: data.city || '',
        country: data.country || '',
        tax_number: data.tax_number || '',
        credit_limit: String(data.credit_limit || 0),
        is_active: data.is_active,
      });
      
    } catch (err: any) {
      setError(err.message || 'Failed to load customer data.');
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const value = e.target.type === 'checkbox' ? (e.target as HTMLInputElement).checked : e.target.value;
    setForm(prev => ({ ...prev, [e.target.name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (!form.name || !form.email || !id) {
      setError('Please fill out all required fields.');
      return;
    }
    
    try {
      setSaving(true);
      await updateCustomer(id, {
        ...form,
        credit_limit: Number(form.credit_limit) || 0,
      });
      navigate('/customers');
    } catch (err: any) {
      setError(err.message || 'Failed to update customer.');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="form-root">
        <div className="form-loading">Loading customer data...</div>
      </div>
    );
  }

  return (
    <div className="form-root">
      <div className="form-header">
        <div className="form-header__text">
          <h1 className="form-title">Edit Customer</h1>
          <p className="form-subtitle">Update customer profile details.</p>
        </div>
        <button className="form-btn form-btn--ghost" onClick={() => navigate('/customers')}>
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
            <label className="form-label">Full Name *</label>
            <input className="form-input" name="name" value={form.name} onChange={handleChange} required />
          </div>
          <div className="form-group">
            <label className="form-label">Email Address *</label>
            <input className="form-input" type="email" name="email" value={form.email} onChange={handleChange} required />
          </div>
          
          <div className="form-group">
            <label className="form-label">Phone Number</label>
            <input className="form-input" type="tel" name="phone" value={form.phone} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Tax Number (VAT/NTN)</label>
            <input className="form-input" name="tax_number" value={form.tax_number} onChange={handleChange} />
          </div>

          <div className="form-group form-group--full">
            <label className="form-label">Street Address</label>
            <input className="form-input" name="address" value={form.address} onChange={handleChange} />
          </div>

          <div className="form-group">
            <label className="form-label">City</label>
            <input className="form-input" name="city" value={form.city} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label className="form-label">Country</label>
            <input className="form-input" name="country" value={form.country} onChange={handleChange} />
          </div>

          <div className="form-group">
            <label className="form-label">Credit Limit ($)</label>
            <input className="form-input" type="number" step="0.01" name="credit_limit" value={form.credit_limit} onChange={handleChange} placeholder="0.00" />
          </div>
          
          <div className="form-group" style={{ flexDirection: 'row', alignItems: 'center', gap: '10px', alignSelf: 'flex-end', height: '42px' }}>
            <input 
              type="checkbox" 
              name="is_active" 
              id="is_active"
              checked={form.is_active} 
              onChange={handleChange} 
              style={{ width: '18px', height: '18px' }}
            />
            <label htmlFor="is_active" className="form-label" style={{ cursor: 'pointer', margin: 0 }}>Active Customer</label>
          </div>
        </div>

        <div className="form-footer">
          <button type="button" className="form-btn form-btn--ghost" onClick={() => navigate('/customers')} disabled={saving}>
            Cancel
          </button>
          <button type="submit" className="form-btn form-btn--primary" disabled={saving}>
            {saving ? 'Updating...' : 'Update Customer'}
          </button>
        </div>
      </form>
    </div>
  );
}
