initialize_supabase() {
  npx supabase init
}

start_supabase() {
  npx supabase start
  npx supabase status
  npx supabase stop
}