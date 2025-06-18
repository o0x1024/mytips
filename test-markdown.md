# Markdown 渲染测试

这是一个测试文件，用于验证新的 marked + Prism 渲染功能。

## 代码高亮测试

### JavaScript 代码
```javascript
function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

// 测试函数
console.log(fibonacci(10)); // 输出: 55
```

### Python 代码
```python
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)

# 测试
print(quicksort([3, 6, 8, 10, 1, 2, 1]))
```

### TypeScript 代码
```typescript
interface User {
  id: number;
  name: string;
  email: string;
}

class UserService {
  private users: User[] = [];
  
  addUser(user: User): void {
    this.users.push(user);
  }
  
  getUserById(id: number): User | undefined {
    return this.users.find(user => user.id === id);
  }
}
```

### CSS 代码
```css
.code-block-container {
  position: relative;
  margin: 1rem 0;
  border-radius: 0.375rem;
  overflow: hidden;
  background: var(--prism-background, #f5f5f5);
  border: 1px solid var(--prism-border, #e0e0e0);
}

.copy-code-btn {
  padding: 0.25rem;
  border: none;
  background: transparent;
  cursor: pointer;
  transition: all 0.2s ease;
}
```

## 其他 Markdown 功能测试

### 列表
- 无序列表项 1
- 无序列表项 2
  - 嵌套项 1
  - 嵌套项 2
- 无序列表项 3

1. 有序列表项 1
2. 有序列表项 2
3. 有序列表项 3

### 任务列表
- [x] 已完成的任务
- [ ] 未完成的任务
- [x] 另一个已完成的任务

### 引用块
> 这是一个普通的引用块
> 
> 可以包含多行内容

### 表格
| 功能 | 状态 | 备注 |
|------|------|------|
| marked 渲染 | ✅ | 已实现 |
| Prism 高亮 | ✅ | 已实现 |
| 行号显示 | ✅ | 已实现 |
| 复制按钮 | ✅ | 已实现 |

### 链接和图片
[GitHub](https://github.com)

![测试图片](https://via.placeholder.com/300x200)

### 格式化文本
**粗体文本**
*斜体文本*
~~删除线文本~~
`行内代码`

这个测试文件应该能够验证所有新实现的功能。 