BEGIN TRANSACTION;
ALTER TABLE public.oauth DROP CONSTRAINT IF EXISTS oauthtype; -- drop the constraint if already exists
ALTER TABLE public.oauth ADD CONSTRAINT oauthtype CHECK (type IN ('Telegram', 'GitHub', 'AOSC'));
ALTER TABLE public."user" ALTER COLUMN username SET NOT NULL;
ALTER TABLE public."user" ALTER COLUMN "admin" SET NOT NULL;
ALTER TABLE public.request DROP CONSTRAINT IF EXISTS request_packager_id_fkey; -- drop the constraint if already exists
UPDATE request SET packager_id = NULL WHERE packager_id = 0;
ALTER TABLE public.request ADD CONSTRAINT request_packager_id_fkey FOREIGN KEY (packager_id) REFERENCES "user"(id);
ALTER TABLE public.request ALTER COLUMN "status" SET NOT NULL;
ALTER TABLE public.request ALTER COLUMN "type" SET NOT NULL;
ALTER TABLE public.request ALTER COLUMN "name" SET NOT NULL;
ALTER TABLE public.request ALTER COLUMN "requester_id" SET NOT NULL;
ALTER TABLE public.request ALTER COLUMN "pub_date" SET NOT NULL;
END TRANSACTION;
