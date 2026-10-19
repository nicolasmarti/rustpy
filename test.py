from rustpy import Node, Expression

print("############")
print(",".join([ x for x in dir(Node) if not "__" in x]))

left_child = Node(12)
right_child = Node(120)

root = Node(20, left_child, right_child)

root.left.value = -5

print(root.sum_tree())
print(root, type(root))


print("############")
print(",".join([ x for x in dir(Expression) if not "__" in x]))

num1 = Expression.number(10.0)
num2 = Expression.number(20.0)
print(num1, type(num1))

expr = Expression.binaryop("*", num1, num2)
print(expr, type(expr))

result = expr.eval()
print(result)

def s(expr: Expression):
    match (expr):
        case Expression.BinaryOp(op, lhs, rhs):
            return f"{s(lhs)} {op} {s(rhs)}"
        case Expression.Number(n):
            return f"{n}"

print(s(expr))
