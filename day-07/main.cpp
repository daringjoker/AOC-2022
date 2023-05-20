#include <bits/stdc++.h>

using namespace std;

typedef struct Tree_Node {
  bool isDir;
  string name;
  unsigned int size = 0;
  vector<Tree_Node> children;
  Tree_Node* parent;
} Tree_Node;

class Tree {
 private:
  Tree_Node* current;

  void show_dir_content(Tree_Node curr, unsigned int indent) {
    for (auto&& i : curr.children) {

      cout << string(indent, ' ') << "- " << i.name << " ("
           << (i.isDir ? "dir " : "file ") << i.size << ")\n";
      if (i.isDir) {
        show_dir_content(i, indent + 4);
      }
    }
  }

 public:
  Tree_Node root;

  Tree() {
    root.name = "root";
    root.isDir = true;
    root.parent = nullptr;

    current = &root;
  }

  void go_up() {
    // cout << "changing to parent directory" << endl;
    current = current->parent;
  }

  void addChildDir(const string& name) {
    // cout << "adding child directory " << name << endl;
    Tree_Node child;
    child.name = name;
    child.isDir = true;
    child.parent = current;
    child.size = 0;
    current->children.push_back(child);
    this->current = &current->children.at(current->children.size() - 1);
  }

  void show() { show_dir_content(root, 0); }

  void addChildFile(const string& name, unsigned int size) {
    // cout << "adding child file " << name << endl;

    Tree_Node child;
    child.name = name;
    child.isDir = false;
    child.parent = current;
    child.size = size;

    current->children.push_back(child);

    do {
      child.parent->size += size;
      child = *child.parent;
    } while (child.parent != nullptr);
  }
};

unsigned int smaller_than_100000(Tree_Node x) {
  if (!x.isDir)
    return 0;

  unsigned int delta = (x.size <= 100000) ? x.size : 0;
  return delta + accumulate(x.children.begin(), x.children.end(), 0,
                            [](int x, const Tree_Node& y) {
                              return x + smaller_than_100000(y);
                            });
}

unsigned int bigger_than(Tree_Node x, unsigned int lim, unsigned int m) {
  return accumulate(x.children.begin(), x.children.end(), m,
                    [lim](unsigned int m, Tree_Node y) {
                      if (!y.isDir)
                        return m;
                      if (y.size > lim) {
                        unsigned int new_min = min(m, y.size);
                        return min(new_min, bigger_than(y, lim, new_min));
                      }
                      return m;
                    });
}

unsigned int soln1(const Tree& tree) {
  return smaller_than_100000(tree.root);
}

unsigned int soln2(const Tree& tree) {
  unsigned int root_size = tree.root.children[0].size;
  return bigger_than(tree.root, root_size - 40000000, root_size);
}

int main() {
  Tree tree;
  ifstream infile("input.txt");
  string line;
  while (!infile.eof()) {
    getline(infile, line);

    if (line[0] == '$') {
      string command = line.substr(2, 2);
      if (command == "cd") {
        string dirname = line.substr(5);
        if (dirname == "..") {
          tree.go_up();
        } else {
          tree.addChildDir(dirname);
        }
      }
    } else {
      if (line.substr(0, 3) != "dir") {
        string name;
        unsigned int size;
        stringstream i(line);
        i >> size >> name;
        tree.addChildFile(name, size);
      }
    }
  }
  // tree.show();

  cout << "soln1 = " << soln1(tree) << "\n";
  cout << "soln2 = " << soln2(tree) << "\n";
}
