-- Fix user_role enum by adding any missing values

-- Check and add missing enum values
DO $$
BEGIN
    -- Add sales_user if not exists
    IF NOT EXISTS (SELECT 1 FROM pg_enum WHERE enumtypid = 'user_role'::regtype AND enumlabel = 'sales_user') THEN
        ALTER TYPE user_role ADD VALUE 'sales_user';
    END IF;
    
    -- Add purchase_manager if not exists
    IF NOT EXISTS (SELECT 1 FROM pg_enum WHERE enumtypid = 'user_role'::regtype AND enumlabel = 'purchase_manager') THEN
        ALTER TYPE user_role ADD VALUE 'purchase_manager';
    END IF;
    
    -- Add accountant if not exists
    IF NOT EXISTS (SELECT 1 FROM pg_enum WHERE enumtypid = 'user_role'::regtype AND enumlabel = 'accountant') THEN
        ALTER TYPE user_role ADD VALUE 'accountant';
    END IF;
    
    -- Add reports_viewer if not exists
    IF NOT EXISTS (SELECT 1 FROM pg_enum WHERE enumtypid = 'user_role'::regtype AND enumlabel = 'reports_viewer') THEN
        ALTER TYPE user_role ADD VALUE 'reports_viewer';
    END IF;
    
    -- Add read_only if not exists
    IF NOT EXISTS (SELECT 1 FROM pg_enum WHERE enumtypid = 'user_role'::regtype AND enumlabel = 'read_only') THEN
        ALTER TYPE user_role ADD VALUE 'read_only';
    END IF;
END
$$;
