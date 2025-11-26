import os
import shutil
import glob
import pathlib
import time

def fs_read_file(path, encoding='utf-8'):
    try:
        with open(path, 'r', encoding=encoding) as f:
            return f.read()
    except Exception as e:
        raise IOError(f"Cannot read file: {e}")

def fs_read_lines(path, encoding='utf-8'):
    try:
        with open(path, 'r', encoding=encoding) as f:
            return f.readlines()
    except Exception as e:
        raise IOError(f"Cannot read file: {e}")

def fs_read_bytes(path):
    try:
        with open(path, 'rb') as f:
            return list(f.read())
    except Exception as e:
        raise IOError(f"Cannot read file: {e}")

def fs_write_file(path, content, encoding='utf-8'):
    try:
        with open(path, 'w', encoding=encoding) as f:
            f.write(content)
        return None
    except Exception as e:
        raise IOError(f"Cannot write file: {e}")

def fs_write_lines(path, lines, encoding='utf-8'):
    try:
        with open(path, 'w', encoding=encoding) as f:
            f.writelines(lines)
        return None
    except Exception as e:
        raise IOError(f"Cannot write file: {e}")

def fs_write_bytes(path, data):
    try:
        if isinstance(data, list):
            data = bytes(data)
        with open(path, 'wb') as f:
            f.write(data)
        return None
    except Exception as e:
        raise IOError(f"Cannot write file: {e}")

def fs_append_file(path, content, encoding='utf-8'):
    try:
        with open(path, 'a', encoding=encoding) as f:
            f.write(content)
        return None
    except Exception as e:
        raise IOError(f"Cannot append to file: {e}")

def fs_exists(path):
    return os.path.exists(path)

def fs_is_file(path):
    return os.path.isfile(path)

def fs_is_dir(path):
    return os.path.isdir(path)

def fs_is_link(path):
    return os.path.islink(path)

def fs_delete_file(path):
    try:
        os.remove(path)
        return None
    except Exception as e:
        raise IOError(f"Cannot delete file: {e}")

def fs_delete_dir(path, recursive=False):
    try:
        if recursive:
            shutil.rmtree(path)
        else:
            os.rmdir(path)
        return None
    except Exception as e:
        raise IOError(f"Cannot delete directory: {e}")

def fs_create_dir(path, parents=False):
    try:
        if parents:
            os.makedirs(path, exist_ok=True)
        else:
            os.mkdir(path)
        return None
    except Exception as e:
        raise IOError(f"Cannot create directory: {e}")

def fs_list_dir(path):
    try:
        return os.listdir(path)
    except Exception as e:
        raise IOError(f"Cannot list directory: {e}")

def fs_walk(path):
    try:
        result = []
        for root, dirs, files in os.walk(path):
            result.append([root, dirs, files])
        return result
    except Exception as e:
        raise IOError(f"Cannot walk directory: {e}")

def fs_glob(pattern):
    return glob.glob(pattern)

def fs_copy_file(src, dst):
    try:
        shutil.copy2(src, dst)
        return None
    except Exception as e:
        raise IOError(f"Cannot copy file: {e}")

def fs_copy_dir(src, dst):
    try:
        shutil.copytree(src, dst)
        return None
    except Exception as e:
        raise IOError(f"Cannot copy directory: {e}")

def fs_move(src, dst):
    try:
        shutil.move(src, dst)
        return None
    except Exception as e:
        raise IOError(f"Cannot move: {e}")

def fs_rename(src, dst):
    try:
        os.rename(src, dst)
        return None
    except Exception as e:
        raise IOError(f"Cannot rename: {e}")

def fs_get_size(path):
    try:
        if os.path.isfile(path):
            return os.path.getsize(path)
        elif os.path.isdir(path):
            total = 0
            for dirpath, dirnames, filenames in os.walk(path):
                for filename in filenames:
                    filepath = os.path.join(dirpath, filename)
                    total += os.path.getsize(filepath)
            return total
        return 0
    except Exception as e:
        raise IOError(f"Cannot get size: {e}")

def fs_get_mtime(path):
    try:
        return os.path.getmtime(path)
    except Exception as e:
        raise IOError(f"Cannot get modification time: {e}")

def fs_get_ctime(path):
    try:
        return os.path.getctime(path)
    except Exception as e:
        raise IOError(f"Cannot get creation time: {e}")

def fs_get_atime(path):
    try:
        return os.path.getatime(path)
    except Exception as e:
        raise IOError(f"Cannot get access time: {e}")

def fs_touch(path):
    try:
        pathlib.Path(path).touch()
        return None
    except Exception as e:
        raise IOError(f"Cannot touch file: {e}")

def path_join(*parts):
    return os.path.join(*parts)

def path_split(path):
    return list(os.path.split(path))

def path_dirname(path):
    return os.path.dirname(path)

def path_basename(path):
    return os.path.basename(path)

def path_splitext(path):
    return list(os.path.splitext(path))

def path_abspath(path):
    return os.path.abspath(path)

def path_realpath(path):
    return os.path.realpath(path)

def path_relpath(path, start=None):
    if start:
        return os.path.relpath(path, start)
    return os.path.relpath(path)

def path_normpath(path):
    return os.path.normpath(path)

def path_expanduser(path):
    return os.path.expanduser(path)

def path_expandvars(path):
    return os.path.expandvars(path)

def path_is_absolute(path):
    return os.path.isabs(path)

def fs_get_cwd():
    return os.getcwd()

def fs_change_dir(path):
    try:
        os.chdir(path)
        return None
    except Exception as e:
        raise IOError(f"Cannot change directory: {e}")

def fs_get_home():
    return str(pathlib.Path.home())

def fs_get_temp():
    import tempfile
    return tempfile.gettempdir()

def fs_make_temp_file(suffix='', prefix='tmp', dir=None, text=True):
    import tempfile
    fd, path = tempfile.mkstemp(suffix=suffix, prefix=prefix, dir=dir, text=text)
    os.close(fd)
    return path

def fs_make_temp_dir(suffix='', prefix='tmp', dir=None):
    import tempfile
    return tempfile.mkdtemp(suffix=suffix, prefix=prefix, dir=dir)

def fs_symlink(src, dst):
    try:
        os.symlink(src, dst)
        return None
    except Exception as e:
        raise IOError(f"Cannot create symlink: {e}")

def fs_readlink(path):
    try:
        return os.readlink(path)
    except Exception as e:
        raise IOError(f"Cannot read symlink: {e}")

def fs_chmod(path, mode):
    try:
        os.chmod(path, int(mode))
        return None
    except Exception as e:
        raise IOError(f"Cannot change permissions: {e}")

def fs_stat(path):
    try:
        stat_info = os.stat(path)
        return {
            'size': stat_info.st_size,
            'mtime': stat_info.st_mtime,
            'ctime': stat_info.st_ctime,
            'atime': stat_info.st_atime,
            'mode': stat_info.st_mode,
            'uid': stat_info.st_uid,
            'gid': stat_info.st_gid,
        }
    except Exception as e:
        raise IOError(f"Cannot stat: {e}")

def fs_find_files(directory, pattern='*', recursive=True):
    path_obj = pathlib.Path(directory)
    if recursive:
        return [str(p) for p in path_obj.rglob(pattern)]
    else:
        return [str(p) for p in path_obj.glob(pattern)]

def fs_get_extension(path):
    return pathlib.Path(path).suffix

def fs_get_stem(path):
    return pathlib.Path(path).stem

def fs_with_suffix(path, suffix):
    return str(pathlib.Path(path).with_suffix(suffix))

def fs_with_name(path, name):
    return str(pathlib.Path(path).with_name(name))

FILE_SYSTEM_FUNCTIONS = {
    'fs_read_file': fs_read_file,
    'fs_read_lines': fs_read_lines,
    'fs_read_bytes': fs_read_bytes,
    'fs_write_file': fs_write_file,
    'fs_write_lines': fs_write_lines,
    'fs_write_bytes': fs_write_bytes,
    'fs_append_file': fs_append_file,
    'fs_exists': fs_exists,
    'fs_is_file': fs_is_file,
    'fs_is_dir': fs_is_dir,
    'fs_is_link': fs_is_link,
    'fs_delete_file': fs_delete_file,
    'fs_delete_dir': fs_delete_dir,
    'fs_create_dir': fs_create_dir,
    'fs_list_dir': fs_list_dir,
    'fs_walk': fs_walk,
    'fs_glob': fs_glob,
    'fs_copy_file': fs_copy_file,
    'fs_copy_dir': fs_copy_dir,
    'fs_move': fs_move,
    'fs_rename': fs_rename,
    'fs_get_size': fs_get_size,
    'fs_get_mtime': fs_get_mtime,
    'fs_get_ctime': fs_get_ctime,
    'fs_get_atime': fs_get_atime,
    'fs_touch': fs_touch,
    'path_join': path_join,
    'path_split': path_split,
    'path_dirname': path_dirname,
    'path_basename': path_basename,
    'path_splitext': path_splitext,
    'path_abspath': path_abspath,
    'path_realpath': path_realpath,
    'path_relpath': path_relpath,
    'path_normpath': path_normpath,
    'path_expanduser': path_expanduser,
    'path_expandvars': path_expandvars,
    'path_is_absolute': path_is_absolute,
    'fs_get_cwd': fs_get_cwd,
    'fs_change_dir': fs_change_dir,
    'fs_get_home': fs_get_home,
    'fs_get_temp': fs_get_temp,
    'fs_make_temp_file': fs_make_temp_file,
    'fs_make_temp_dir': fs_make_temp_dir,
    'fs_symlink': fs_symlink,
    'fs_readlink': fs_readlink,
    'fs_chmod': fs_chmod,
    'fs_stat': fs_stat,
    'fs_find_files': fs_find_files,
    'fs_get_extension': fs_get_extension,
    'fs_get_stem': fs_get_stem,
    'fs_with_suffix': fs_with_suffix,
    'fs_with_name': fs_with_name,
}
