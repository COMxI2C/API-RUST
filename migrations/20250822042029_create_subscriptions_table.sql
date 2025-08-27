-- Add migration script here
-- Create the suscriptions Table

CREATE TABLE subscriptions(
    id UUID PRIMARY KEY NOT NULL,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    subscribed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);