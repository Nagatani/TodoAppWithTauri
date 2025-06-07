import { describe, it, expect, vi, beforeEach } from 'vitest';
import { writable } from 'svelte/store';
import { addTodo, todos, type Todo } from './todoStore';

let mockInvokeImplementation: (...args: any[]) => Promise<any> = async () => {};

vi.mock('@tauri-apps/api/core', async (importOriginal) => {
  const actual = await importOriginal() as Record<string, unknown>;
  return {
    ...actual,
    invoke: (...args: any[]) => mockInvokeImplementation(...args),
  };
});

beforeEach(() => {
  todos.set([]);
  mockInvokeImplementation = async (command: string, args?: any) => {
    if (command === 'get_todos') {
      return [];
    }
    return undefined;
  };
  vi.clearAllMocks();
});

describe('todoStore', () => {
  describe('addTodo', () => {
    it('should call invoke with "add_todo" and correct parameters including dueDate', async () => {
      const taskText = 'Test new todo';
      const dueDate = '2024-01-01T10:00';

      const localMockInvoke = vi.fn().mockImplementation(mockInvokeImplementation);
      mockInvokeImplementation = localMockInvoke;

      await addTodo(taskText, dueDate);

      expect(localMockInvoke).toHaveBeenCalledWith('add_todo', {
        task: taskText,
        dueDate: dueDate,
      });
    });

    it('should call invoke with "add_todo" and null for dueDate if not provided', async () => {
      const taskText = 'Test todo without due date';

      const localMockInvoke = vi.fn().mockImplementation(mockInvokeImplementation);
      mockInvokeImplementation = localMockInvoke;

      await addTodo(taskText, null);

      expect(localMockInvoke).toHaveBeenCalledWith('add_todo', {
        task: taskText,
        dueDate: null,
      });
    });

    it('should call loadTodos (which invokes "get_todos") after successfully adding a todo', async () => {
      const taskText = 'Another todo';
      const mockTodosAfterAdd: Todo[] = [{ id: 1, task: taskText, completed: false, dueDate: null }];

      const localMockInvoke = vi.fn().mockImplementation(async (command: string, args?: any) => {
        if (command === 'add_todo') {
          return undefined;
        }
        if (command === 'get_todos') {
          return mockTodosAfterAdd;
        }
      });
      mockInvokeImplementation = localMockInvoke;

      await addTodo(taskText, null);

      expect(localMockInvoke).toHaveBeenCalledWith('add_todo', { task: taskText, dueDate: null });
      expect(localMockInvoke).toHaveBeenCalledWith('get_todos');

      let currentTodos: Todo[] = [];
      const unsubscribe = todos.subscribe(value => { currentTodos = value; });
      expect(currentTodos).toEqual(mockTodosAfterAdd);
      unsubscribe();
    });

    it('should not call invoke for "add_todo" if taskText is empty', async () => {
      const localMockInvoke = vi.fn().mockImplementation(mockInvokeImplementation);
      mockInvokeImplementation = localMockInvoke;
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await addTodo('', null);

      expect(localMockInvoke).not.toHaveBeenCalledWith('add_todo', expect.anything());
      expect(consoleErrorSpy).toHaveBeenCalledWith("Task cannot be empty");
      consoleErrorSpy.mockRestore();
    });

    it('should not call invoke for "add_todo" if taskText is only whitespace', async () => {
      const localMockInvoke = vi.fn().mockImplementation(mockInvokeImplementation);
      mockInvokeImplementation = localMockInvoke;
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await addTodo('   ', null);

      expect(localMockInvoke).not.toHaveBeenCalledWith('add_todo', expect.anything());
      expect(consoleErrorSpy).toHaveBeenCalledWith("Task cannot be empty");
      consoleErrorSpy.mockRestore();
    });

    it('should log an error and not throw if invoke("add_todo") fails, and not call get_todos', async () => {
      const taskText = 'Todo that fails to add';
      const expectedError = new Error('Failed to add');

      const localMockInvoke = vi.fn().mockImplementation(async (command: string, args?: any) => {
        if (command === 'add_todo') {
          throw expectedError;
        }
      });
      mockInvokeImplementation = localMockInvoke;
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await addTodo(taskText, null);

      expect(localMockInvoke).toHaveBeenCalledWith('add_todo', { task: taskText, dueDate: null });
      expect(localMockInvoke).not.toHaveBeenCalledWith('get_todos');
      expect(consoleErrorSpy).toHaveBeenCalledWith("Failed to add todo:", expectedError);

      consoleErrorSpy.mockRestore();
    });
  });
});
