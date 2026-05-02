-- Fix tags column to use JSONB instead of TEXT[] for better JSON handling
ALTER TABLE suppliers ALTER COLUMN tags TYPE JSONB USING tags::text::jsonb;
