function build()
    local f = io.open("solution.c", 'w')
    if f == nil then
        err = "can't open solution.c for write"
        return
    end
    f:write(source_string)
    f:close()
   
    local t = io.popen('gcc solution.c -std=c11 -o solution')
    local e = t:read("*all")
    if e ~= "" then
        err = e
        return
    end
end

function run()
    local f = io.open("input", 'w')
    if f == nil then
        err = "can't open input for write"
        return
    end
    f:write(input)
    f:close()

    local ptime = os.time();
    local t = io.popen('./solution < input')
    local ctime = os.time();

    local output = t:read("*all");
    if output ~= expect then
        err = table.concat({"expect ", expect, ", got ", output});
    end
    return ctime - ptime;
end
