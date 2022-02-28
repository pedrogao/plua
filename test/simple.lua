function gen(n)
   if n < 2 then
      return n;
   end

   local n1 = n + 1;
   local n2 = n + 2;
   return n1 + n2;
end

local r = gen(4);
print(r);