select 
  id,
  name,
  account_id
from user
where (? is null
       or id = ?)
  and (? is null
       or name = ?)
  and (? is null
       or account_id = ?)
  and (? is null
       or hashed_passwd = ?)
limit 1
